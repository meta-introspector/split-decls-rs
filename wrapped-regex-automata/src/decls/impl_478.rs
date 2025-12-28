macro_rules! deps {
    () => {
        Transition!();
        Utf8LastTransition!();
        ThompsonRef!();
        Utf8State!();
        Utf8Node!();
        Utf8Compiler!();
        StateID!();
        Builder!();
        BuildError!();
    };
}

macro_rules! impl_478 {
    () => {
        deps!();
        impl < 'a > Utf8Compiler < 'a > { fn new (builder : & 'a mut Builder , state : & 'a mut Utf8State ,) -> Result < Utf8Compiler < 'a > , BuildError > { let target = builder . add_empty () ? ; state . clear () ; let mut utf8c = Utf8Compiler { builder , state , target } ; utf8c . add_empty () ; Ok (utf8c) } fn finish (& mut self) -> Result < ThompsonRef , BuildError > { self . compile_from (0) ? ; let node = self . pop_root () ; let start = self . compile (node) ? ; Ok (ThompsonRef { start , end : self . target }) } fn add (& mut self , ranges : & [Utf8Range]) -> Result < () , BuildError > { let prefix_len = ranges . iter () . zip (& self . state . uncompiled) . take_while (| & (range , node) | { node . last . as_ref () . map_or (false , | t | { (t . start , t . end) == (range . start , range . end) }) }) . count () ; assert ! (prefix_len < ranges . len ()) ; self . compile_from (prefix_len) ? ; self . add_suffix (& ranges [prefix_len ..]) ; Ok (()) } fn compile_from (& mut self , from : usize) -> Result < () , BuildError > { let mut next = self . target ; while from + 1 < self . state . uncompiled . len () { let node = self . pop_freeze (next) ; next = self . compile (node) ? ; } self . top_last_freeze (next) ; Ok (()) } fn compile (& mut self , node : Vec < Transition > ,) -> Result < StateID , BuildError > { let hash = self . state . compiled . hash (& node) ; if let Some (id) = self . state . compiled . get (& node , hash) { return Ok (id) ; } let id = self . builder . add_sparse (node . clone ()) ? ; self . state . compiled . set (node , hash , id) ; Ok (id) } fn add_suffix (& mut self , ranges : & [Utf8Range]) { assert ! (! ranges . is_empty ()) ; let last = self . state . uncompiled . len () . checked_sub (1) . expect ("non-empty nodes") ; assert ! (self . state . uncompiled [last] . last . is_none ()) ; self . state . uncompiled [last] . last = Some (Utf8LastTransition { start : ranges [0] . start , end : ranges [0] . end , }) ; for r in & ranges [1 ..] { self . state . uncompiled . push (Utf8Node { trans : vec ! [] , last : Some (Utf8LastTransition { start : r . start , end : r . end }) , }) ; } } fn add_empty (& mut self) { self . state . uncompiled . push (Utf8Node { trans : vec ! [] , last : None }) ; } fn pop_freeze (& mut self , next : StateID) -> Vec < Transition > { let mut uncompiled = self . state . uncompiled . pop () . unwrap () ; uncompiled . set_last_transition (next) ; uncompiled . trans } fn pop_root (& mut self) -> Vec < Transition > { assert_eq ! (self . state . uncompiled . len () , 1) ; assert ! (self . state . uncompiled [0] . last . is_none ()) ; self . state . uncompiled . pop () . expect ("non-empty nodes") . trans } fn top_last_freeze (& mut self , next : StateID) { let last = self . state . uncompiled . len () . checked_sub (1) . expect ("non-empty nodes") ; self . state . uncompiled [last] . set_last_transition (next) ; } }
    };
}

impl_478!();