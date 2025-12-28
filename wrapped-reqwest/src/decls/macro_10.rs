macro_rules! macro_10 {
    () => {
        if_wasm ! { mod wasm ; mod util ; pub use self :: wasm :: { Body , Client , ClientBuilder , Request , RequestBuilder , Response } ; # [cfg (feature = "multipart")] pub use self :: wasm :: multipart ; }
    };
}

macro_10!()