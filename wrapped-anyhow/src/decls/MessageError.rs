macro_rules! MessageError {
    () => {
        # [repr (transparent)] pub struct MessageError < M > (pub M) ;
    };
}

MessageError!();