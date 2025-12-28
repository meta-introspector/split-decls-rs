macro_rules! deps {
    () => {
        DemangleWrite!();
        Type!();
        Result!();
        Demangle!();
        FunctionType!();
        ArrayType!();
        PointerToMemberType!();
        ArgScopeStack!();
        DemangleContext!();
    };
}

macro_rules! DemangleAsInner {
    () => {
        deps!();
        # [doc = " Any AST node that can be printed as an inner type."] # [doc = ""] # [doc = " See the comments surrounding `DemangleContext::inner` for details."] # [doc (hidden)] pub trait DemangleAsInner < 'subs , W > : Demangle < 'subs , W > where W : 'subs + DemangleWrite , { # [doc = " Write the inner demangling form of this AST node to the given context."] fn demangle_as_inner < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { self . demangle (ctx , scope) } # [doc = " Cast this `DemangleAsInner` to a `Type`."] fn downcast_to_type (& self) -> Option < & Type > { None } # [doc = " Cast this `DemangleAsInner` to a `FunctionType`."] fn downcast_to_function_type (& self) -> Option < & FunctionType > { None } # [doc = " Cast this `DemangleAsInner` to an `ArrayType`."] fn downcast_to_array_type (& self) -> Option < & ArrayType > { None } # [doc = " Cast this `DemangleAsInner` to a `PointerToMember`."] fn downcast_to_pointer_to_member (& self) -> Option < & PointerToMemberType > { None } fn is_qualified (& self) -> bool { false } }
    };
}

DemangleAsInner!();