macro_rules! deps {
    () => {
        DerivedFieldMetadata!();
        SimpleObjectField!();
    };
}

macro_rules! SimpleObjectFieldGenerator {
    () => {
        deps!();
        struct SimpleObjectFieldGenerator < 'a > { field : & 'a SimpleObjectField , derived : Option < DerivedFieldMetadata > , }
    };
}

SimpleObjectFieldGenerator!();