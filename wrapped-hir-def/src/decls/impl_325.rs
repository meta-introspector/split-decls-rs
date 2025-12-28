macro_rules! deps {
    () => {
        ModuleDefId!();
        ScopeDef!();
        MacroId!();
        BindingId!();
        PerNs!();
        ScopeNames!();
    };
}

macro_rules! impl_325 {
    () => {
        deps!();
        impl ScopeNames { fn add (& mut self , name : & Name , def : ScopeDef) { let set = self . map . entry (name . clone ()) . or_default () ; if ! set . contains (& def) { set . push (def) } } fn add_per_ns (& mut self , name : & Name , def : PerNs) { if let Some (ty) = & def . types { self . add (name , ScopeDef :: ModuleDef (ty . def)) } if let Some (def) = & def . values { self . add (name , ScopeDef :: ModuleDef (def . def)) } if let Some (mac) = & def . macros { self . add (name , ScopeDef :: ModuleDef (ModuleDefId :: MacroId (mac . def))) } if def . is_none () { self . add (name , ScopeDef :: Unknown) } } fn add_local (& mut self , name : & Name , binding : BindingId) { let set = self . map . entry (name . clone ()) . or_default () ; if set . iter () . any (| it | matches ! (it , & ScopeDef :: Local (_))) { cov_mark :: hit ! (shadowing_shows_single_completion) ; return ; } set . push (ScopeDef :: Local (binding)) } }
    };
}

impl_325!()