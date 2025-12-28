macro_rules! DeserializeStateError {
    () => {
        # [doc = " The error type returned when an object cannot be deserialized from the state."] # [derive (Copy , Clone , Eq , PartialEq , Debug)] pub struct DeserializeStateError ;
    };
}

DeserializeStateError!();