use rustc_span::hygiene::LocalExpnId;

pub trait DeriveResolutionProvider<D> {
    fn take_derive_resolutions(&mut self, expn_id: LocalExpnId) -> Option<Vec<D>>;
}

pub trait OpaqueDeriveResolution {}

pub trait ImportResolver {}