macro_rules! deps {
    () => {
        ModuleItemMap!();
        Scope!();
    };
}

macro_rules! Resolver {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct Resolver < 'db > { # [doc = " The stack of scopes, where the inner-most scope is the last item."] # [doc = ""] # [doc = " When using, you generally want to process the scopes in reverse order,"] # [doc = " there's `scopes` *method* for that."] scopes : Vec < Scope < 'db > > , module_scope : ModuleItemMap < 'db > , }
    };
}

Resolver!();