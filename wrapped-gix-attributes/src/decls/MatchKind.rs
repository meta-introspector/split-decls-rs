macro_rules! deps {
    () => {
        AttributeId!();
    };
}

macro_rules! MatchKind {
    () => {
        deps!();
        # [doc = " The kind of attribute within the context of a [match][Match]."] # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash , Ord , PartialOrd)] pub enum MatchKind { # [doc = " A attribute."] Attribute { # [doc = " The location of the macro which referred to it the list with all in-order attributes and macros, or `None` if"] # [doc = " this is attribute wasn't resolved."] # [doc = ""] # [doc = " Use [`Outcome::match_by_id()`] to retrieve the macro."] macro_id : Option < AttributeId > , } , # [doc = " The attribute is a macro, which will resolve into one or more attributes or macros."] Macro { # [doc = " The location of the parent macro which referred to this one in the list with all in-order attributes and macros,"] # [doc = " or `None` if this is macro wasn't resolved by another one."] # [doc = ""] # [doc = " Use [`Outcome::match_by_id()`] to retrieve the parent."] parent_macro_id : Option < AttributeId > , } , }
    };
}

MatchKind!();