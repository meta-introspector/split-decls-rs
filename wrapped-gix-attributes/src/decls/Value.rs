macro_rules! deps {
    () => {
        AttributeId!();
        Assignments!();
    };
}

macro_rules! Value {
    () => {
        deps!();
        # [doc = " A value of a [pattern mapping][gix_glob::search::pattern::Mapping],"] # [doc = " which is either a macro definition or a set of attributes."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] pub enum Value { # [doc = " A macro, whose name resolves to the contained assignments. Note that the name is the pattern of the mapping itself."] MacroAssignments { # [doc = " The id of the macro itself, which is both an attribute as well as a set of additional attributes into which the macro"] # [doc = " resolves"] id : AttributeId , # [doc = " The attributes or assignments that the macro resolves to."] assignments : Assignments , } , # [doc = " A set of assignments which are the attributes themselves."] Assignments (Assignments) , }
    };
}

Value!();