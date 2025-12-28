macro_rules! deps {
    () => {
        SiblingLocation!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl SiblingLocation { fn merge (& mut self , other : SiblingLocation) { use SiblingLocation :: * ; * self = match (* self , other) { (any , NotFound) => any , (NotFound , any) => any , (Above , Below) => AboveAndBelow , (Below , Above) => AboveAndBelow , (AboveAndBelow , _) => AboveAndBelow , (_ , AboveAndBelow) => AboveAndBelow , (Above , Above) => Above , (Below , Below) => Below , } ; } }
    };
}

impl_147!();