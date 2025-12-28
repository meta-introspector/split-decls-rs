macro_rules! deps {
    () => {
        AList!();
    };
}

macro_rules! AttrList {
    () => {
        deps!();
        # [doc = " A list of `AList`s, i.e. a list of list of attributes. This (strange)"] # [doc = " indirection is induced by the grammar. This structure corresponds to the"] # [doc = " `attr_list` non-terminal of the grammar."] # [doc = ""] # [doc = " Notice methods `flatten` and `flatten_ref` to remove the indirection."] # [derive (Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct AttrList < A > { # [doc = " The list of `AList`s."] pub elems : Vec < AList < A > > , }
    };
}

AttrList!()