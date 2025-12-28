macro_rules! DisplayError {
    () => {
        # [repr (transparent)] pub struct DisplayError < M > (pub M) ;
    };
}

DisplayError!()