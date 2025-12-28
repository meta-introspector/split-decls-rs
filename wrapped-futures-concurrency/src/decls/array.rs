macro_rules! deps {
    () => {
        TryJoin!();
        Join!();
        Merge!();
        Zip!();
        RaceOk!();
        Race!();
        Chain!();
        AggregateError!();
    };
}

macro_rules! array {
    () => {
        deps!();
        # [doc = " Helper functions and types for fixed-length arrays."] pub mod array { pub use crate :: future :: join :: array :: Join ; pub use crate :: future :: race :: array :: Race ; pub use crate :: future :: race_ok :: array :: { AggregateError , RaceOk } ; pub use crate :: future :: try_join :: array :: TryJoin ; pub use crate :: stream :: chain :: array :: Chain ; pub use crate :: stream :: merge :: array :: Merge ; pub use crate :: stream :: zip :: array :: Zip ; }
    };
}

array!()