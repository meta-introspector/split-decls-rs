macro_rules! deps {
    () => {
        UnqualifiedName!();
        UnscopedTemplateName!();
        UnscopedName!();
        TemplateArgs!();
        Name!();
    };
}

macro_rules! MemberName {
    () => {
        deps!();
        # [doc = " In libiberty, Member and DerefMember expressions have special handling."] # [doc = " They parse an `UnqualifiedName` (not an `UnscopedName` as the cxxabi docs"] # [doc = " say) and optionally a `TemplateArgs` if it is present. We can't just parse"] # [doc = " a `Name` or an `UnscopedTemplateName` here because that allows other inputs"] # [doc = " that libiberty does not."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct MemberName (Name) ;
    };
}

MemberName!();