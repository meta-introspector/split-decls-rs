macro_rules! PointerToMemberType {
    () => {
        # [doc = " The `<pointer-to-member-type>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <pointer-to-member-type> ::= M <class type> <member type>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct PointerToMemberType (TypeHandle , TypeHandle) ;
    };
}

PointerToMemberType!()