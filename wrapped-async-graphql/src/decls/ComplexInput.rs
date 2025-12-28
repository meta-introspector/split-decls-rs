macro_rules! deps {
    () => {
        InputObject!();
    };
}

macro_rules! ComplexInput {
    () => {
        deps!();
        # [derive (InputObject)] # [graphql (internal)] struct ComplexInput { required_field : bool , int_field : Option < i32 > , string_field : Option < String > , boolean_field : Option < bool > , string_list_field : Option < Vec < Option < String > > > , }
    };
}

ComplexInput!()