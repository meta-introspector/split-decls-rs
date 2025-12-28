macro_rules! deps {
    () => {
        ConstBuffer!();
    };
}

macro_rules! RuntimeType {
    () => {
        deps!();
        # [doc (hidden)] pub trait RuntimeType : Type < Self > { const SIGNATURE : imp :: ConstBuffer ; }
    };
}

RuntimeType!();