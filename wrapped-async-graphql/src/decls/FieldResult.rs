macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! FieldResult {
    () => {
        deps!();
        # [doc = " An alias of [async_graphql::Result](type.Result.html). Present for backward"] # [doc = " compatibility reasons."] pub type FieldResult < T > = Result < T > ;
    };
}

FieldResult!();