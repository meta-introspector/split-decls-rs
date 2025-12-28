macro_rules! deps {
    () => {
        SimpleObjectField!();
        DerivedFieldMetadata!();
    };
}

macro_rules! SimpleObjectFieldGenerator {
    () => {
        deps!();
        struct SimpleObjectFieldGenerator < 'a > { field : & 'a SimpleObjectField , derived : Option < DerivedFieldMetadata > , }
    };
}

SimpleObjectFieldGenerator!()