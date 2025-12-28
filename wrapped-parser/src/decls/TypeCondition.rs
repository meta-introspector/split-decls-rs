macro_rules! deps {
    () => {
        Positioned!();
    };
}

macro_rules! TypeCondition {
    () => {
        deps!();
        # [doc = " A type a fragment can apply to (`on` followed by the type)."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#TypeCondition)."] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TypeCondition { # [doc = " The type this fragment applies to."] pub on : Positioned < Name > , }
    };
}

TypeCondition!()