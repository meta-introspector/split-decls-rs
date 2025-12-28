macro_rules! deps {
    () => {
        OpenError!();
    };
}

macro_rules! reveal_fallback {
    () => {
        deps!();
        # [cfg (feature = "reveal")] fn reveal_fallback (path : & std :: path :: Path) -> Result < () , OpenError > { let path = path . canonicalize () . map_err (OpenError :: Io) ? ; let parent = path . parent () . unwrap_or (std :: path :: Path :: new ("/")) ; open (parent . as_os_str ()) }
    };
}

reveal_fallback!()