macro_rules! deps {
    () => {
        UnwindTable!();
        Section!();
        CommonInformationEntry!();
        Result!();
        UnwindContextStorage!();
        UnwindContext!();
        RegisterRule!();
        ReaderOffset!();
        Register!();
        Error!();
        Reader!();
        UnwindSection!();
        CfaRule!();
        UnwindTableRow!();
        BaseAddresses!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        # [doc = " # Signal Safe Methods"] # [doc = ""] # [doc = " These methods are guaranteed not to allocate, acquire locks, or perform any"] # [doc = " other signal-unsafe operations, if an non-allocating storage is used."] impl < T , S > UnwindContext < T , S > where T : ReaderOffset , S : UnwindContextStorage < T > , { # [doc = " Construct a new call frame unwinding context."] pub fn new_in () -> Self { let mut ctx = UnwindContext { stack : Default :: default () , initial_rule : None , is_initialized : false , } ; ctx . reset () ; ctx } # [doc = " Run the CIE's initial instructions and initialize this `UnwindContext`."] fn initialize < Section , R > (& mut self , section : & Section , bases : & BaseAddresses , cie : & CommonInformationEntry < R > ,) -> Result < () > where R : Reader < Offset = T > , Section : UnwindSection < R > , { self . reset () ; let mut table = UnwindTable :: new_for_cie (section , bases , self , cie) ; while table . next_row () ? . is_some () { } self . save_initial_rules () ? ; Ok (()) } fn reset (& mut self) { self . stack . clear () ; self . stack . try_push (UnwindTableRow :: default ()) . unwrap () ; debug_assert ! (self . stack [0] . is_default ()) ; self . initial_rule = None ; self . is_initialized = false ; } fn row (& self) -> & UnwindTableRow < T , S > { self . stack . last () . unwrap () } fn row_mut (& mut self) -> & mut UnwindTableRow < T , S > { self . stack . last_mut () . unwrap () } fn save_initial_rules (& mut self) -> Result < () > { debug_assert ! (! self . is_initialized) ; self . initial_rule = match * self . stack . last () . unwrap () . registers . rules { [] => Some ((Register (0) , RegisterRule :: Undefined)) , [ref rule] => Some (rule . clone ()) , _ => { let rules = self . stack . last () . unwrap () . clone () ; self . stack . try_insert (0 , rules) . map_err (| _ | Error :: StackFull) ? ; None } } ; self . is_initialized = true ; Ok (()) } fn start_address (& self) -> u64 { self . row () . start_address } fn set_start_address (& mut self , start_address : u64) { let row = self . row_mut () ; row . start_address = start_address ; } fn set_register_rule (& mut self , register : Register , rule : RegisterRule < T >) -> Result < () > { let row = self . row_mut () ; row . registers . set (register , rule) } # [doc = " Returns `None` if we have not completed evaluation of a CIE's initial"] # [doc = " instructions."] fn get_initial_rule (& self , register : Register) -> Option < RegisterRule < T > > { if ! self . is_initialized { return None ; } Some (match self . initial_rule { None => self . stack [0] . registers . get (register) , Some ((r , ref rule)) if r == register => rule . clone () , _ => RegisterRule :: Undefined , }) } fn set_cfa (& mut self , cfa : CfaRule < T >) { self . row_mut () . cfa = cfa ; } fn cfa_mut (& mut self) -> & mut CfaRule < T > { & mut self . row_mut () . cfa } fn push_row (& mut self) -> Result < () > { let new_row = self . row () . clone () ; self . stack . try_push (new_row) . map_err (| _ | Error :: StackFull) } fn pop_row (& mut self) -> Result < () > { let min_size = if self . is_initialized && self . initial_rule . is_none () { 2 } else { 1 } ; if self . stack . len () <= min_size { return Err (Error :: PopWithEmptyStack) ; } self . stack . pop () . unwrap () ; Ok (()) } }
    };
}

impl_218!()