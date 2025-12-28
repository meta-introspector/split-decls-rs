macro_rules! BackendTypes {
    () => {
        pub trait BackendTypes { type Value : CodegenObject + PartialEq ; type Metadata : CodegenObject ; type Function : CodegenObject ; type BasicBlock : Copy ; type Type : CodegenObject + PartialEq ; type Funclet ; type DIScope : Copy + Hash + PartialEq + Eq ; type DILocation : Copy ; type DIVariable : Copy ; }
    };
}

BackendTypes!()