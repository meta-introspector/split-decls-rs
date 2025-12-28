macro_rules! deps {
    () => {
        Error!();
        UnwindContextStorage!();
        Register!();
        Result!();
        RegisterRuleMap!();
        RegisterRule!();
        RegisterRuleIter!();
        ReaderOffset!();
    };
}

macro_rules! impl_225 {
    () => {
        deps!();
        # [doc = " # Signal Safe Methods"] # [doc = ""] # [doc = " These methods are guaranteed not to allocate, acquire locks, or perform any"] # [doc = " other signal-unsafe operations."] impl < T , S > RegisterRuleMap < T , S > where T : ReaderOffset , S : UnwindContextStorage < T > , { fn is_default (& self) -> bool { self . rules . is_empty () } fn get (& self , register : Register) -> RegisterRule < T > { self . rules . iter () . find (| rule | rule . 0 == register) . map (| r | { debug_assert ! (r . 1 . is_defined ()) ; r . 1 . clone () }) . unwrap_or (RegisterRule :: Undefined) } fn set (& mut self , register : Register , rule : RegisterRule < T >) -> Result < () > { if ! rule . is_defined () { let idx = self . rules . iter () . enumerate () . find (| & (_ , r) | r . 0 == register) . map (| (i , _) | i) ; if let Some (idx) = idx { self . rules . swap_remove (idx) ; } return Ok (()) ; } for & mut (reg , ref mut old_rule) in & mut * self . rules { debug_assert ! (old_rule . is_defined ()) ; if reg == register { * old_rule = rule ; return Ok (()) ; } } self . rules . try_push ((register , rule)) . map_err (| _ | Error :: TooManyRegisterRules) } fn iter (& self) -> RegisterRuleIter < '_ , T > { RegisterRuleIter (self . rules . iter ()) } }
    };
}

impl_225!();