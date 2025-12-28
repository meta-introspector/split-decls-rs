macro_rules! deps {
    () => {
        OnUpgrade!();
    };
}

macro_rules! on {
    () => {
        deps!();
        # [doc = " Gets a pending HTTP upgrade from this message."] # [doc = ""] # [doc = " This can be called on the following types:"] # [doc = ""] # [doc = " - `http::Request<B>`"] # [doc = " - `http::Response<B>`"] # [doc = " - `&mut http::Request<B>`"] # [doc = " - `&mut http::Response<B>`"] pub fn on < T : sealed :: CanUpgrade > (msg : T) -> OnUpgrade { msg . on_upgrade () }
    };
}

on!();