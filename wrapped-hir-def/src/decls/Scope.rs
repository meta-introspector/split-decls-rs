macro_rules! deps {
    () => {
        GenericDefId!();
        ModuleItemMap!();
        ExprScope!();
    };
}

macro_rules! Scope {
    () => {
        deps!();
        # [derive (Debug , Clone)] enum Scope < 'db > { # [doc = " All the items and imported names of a module"] BlockScope (ModuleItemMap < 'db >) , # [doc = " Brings the generic parameters of an item into scope as well as the `Self` type alias /"] # [doc = " generic for ADTs and impls."] GenericParams { def : GenericDefId , params : Arc < GenericParams > } , # [doc = " Local bindings"] ExprScope (ExprScope) , # [doc = " Macro definition inside bodies that affects all paths after it in the same block."] MacroDefScope (MacroDefId) , }
    };
}

Scope!();