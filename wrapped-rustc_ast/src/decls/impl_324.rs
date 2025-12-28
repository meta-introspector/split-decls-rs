macro_rules! deps {
    () => {
        AutoDiffAttrs!();
        DiffMode!();
        DiffActivity!();
        AutoDiffItem!();
    };
}

macro_rules! impl_324 {
    () => {
        deps!();
        impl AutoDiffAttrs { pub fn has_ret_activity (& self) -> bool { self . ret_activity != DiffActivity :: None } pub fn has_active_only_ret (& self) -> bool { self . ret_activity == DiffActivity :: ActiveOnly } pub const fn error () -> Self { AutoDiffAttrs { mode : DiffMode :: Error , width : 0 , ret_activity : DiffActivity :: None , input_activity : Vec :: new () , } } pub fn source () -> Self { AutoDiffAttrs { mode : DiffMode :: Source , width : 0 , ret_activity : DiffActivity :: None , input_activity : Vec :: new () , } } pub fn is_active (& self) -> bool { self . mode != DiffMode :: Error } pub fn is_source (& self) -> bool { self . mode == DiffMode :: Source } pub fn apply_autodiff (& self) -> bool { ! matches ! (self . mode , DiffMode :: Error | DiffMode :: Source) } pub fn into_item (self , source : String , target : String) -> AutoDiffItem { AutoDiffItem { source , target , attrs : self } } }
    };
}

impl_324!()