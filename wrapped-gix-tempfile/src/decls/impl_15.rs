macro_rules! deps {
    () => {
        Closed!();
        ContainingDirectory!();
        AutoRemove!();
        Mode!();
        ForksafeTempfile!();
        Handle!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        # [doc = " Creation and ownership transfer"] impl Handle < Closed > { # [doc = " Create a registered tempfile at the given `path`, where `path` includes the desired filename and close it immediately."] # [doc = ""] # [doc = " Depending on the `directory` configuration, intermediate directories will be created, and depending on `cleanup` empty"] # [doc = " intermediate directories will be removed."] # [doc = ""] # [doc = " ### Warning of potential leaks"] # [doc = ""] # [doc = " Without [signal handlers](crate::signal) installed, tempfiles will remain once a termination"] # [doc = " signal is encountered as destructors won't run. See [the top-level documentation](crate) for more."] pub fn at (path : impl AsRef < Path > , directory : ContainingDirectory , cleanup : AutoRemove) -> io :: Result < Self > { Ok (Handle { id : Handle :: < () > :: at_path (path . as_ref () , directory , cleanup , Mode :: Closed , None) ? , _marker : Default :: default () , }) } # [doc = " Like [`at`](Self::at()), but with support for filesystem `permissions`."] pub fn at_with_permissions (path : impl AsRef < Path > , directory : ContainingDirectory , cleanup : AutoRemove , permissions : std :: fs :: Permissions ,) -> io :: Result < Self > { Ok (Handle { id : Handle :: < () > :: at_path (path . as_ref () , directory , cleanup , Mode :: Closed , Some (permissions)) ? , _marker : Default :: default () , }) } # [doc = " Take ownership of the temporary file path, which deletes it when dropped without persisting it beforehand."] # [doc = ""] # [doc = " It's a theoretical possibility that the file isn't present anymore if signals interfere, hence the `Option`"] pub fn take (self) -> Option < TempPath > { let res = REGISTRY . remove (& self . id) ; std :: mem :: forget (self) ; res . and_then (| (_k , v) | v . map (ForksafeTempfile :: into_temppath)) } }
    };
}

impl_15!()