macro_rules! macro_183 {
    () => {
        pin_project ! { # [doc = " Future for the [`StreamExt::unzip()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct UnzipFuture < S , FromA , FromB > { # [pin] stream : S , res : Option < (FromA , FromB) >, } }
    };
}

macro_183!();