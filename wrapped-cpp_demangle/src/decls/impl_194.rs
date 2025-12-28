macro_rules! deps {
    () => {
        Result!();
        DemangleContext!();
        ArgScopeStack!();
        DemangleAsInner!();
        DemangleWrite!();
        BareFunctionType!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl < 'subs , W > DemangleAsInner < 'subs , W > for BareFunctionType where W : 'subs + DemangleWrite , { fn demangle_as_inner < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle_as_inner ! (self , ctx , scope) ; self . args () . demangle_as_inner (ctx , scope) ? ; Ok (()) } }
    };
}

impl_194!();