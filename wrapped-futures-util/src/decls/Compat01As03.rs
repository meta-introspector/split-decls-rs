macro_rules! Compat01As03 {
    () => {
        # [doc = " Converts a futures 0.1 Future, Stream, AsyncRead, or AsyncWrite"] # [doc = " object to a futures 0.3-compatible version,"] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Compat01As03 < T > { pub (crate) inner : Spawn01 < T > , }
    };
}

Compat01As03!();