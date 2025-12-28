macro_rules! ExtractError {
    () => {
        # [derive (Debug)] pub enum ExtractError < T > { # [doc = " The extracted message was of a different method than expected."] MethodMismatch (T) , # [doc = " Failed to deserialize the message."] JsonError { method : String , error : serde_json :: Error } , }
    };
}

ExtractError!()