// Generated macro for impl_86 (impl)
macro_rules! Depcrate_header_mapimpl_86 {
() => {
// Module: crate::header::map
// Provides: {"impl_86"}
// Dependencies: {}
impl < 'a , T > Iterator for Drain < 'a , T > { type Item = (Option < HeaderName > , T) ; fn next (& mut self) -> Option < Self :: Item > { if let Some (next) = self . next { let raw_links = RawLinks (self . entries) ; let extra = unsafe { remove_extra_value (raw_links , & mut * self . extra_values , next) } ; match extra . next { Link :: Extra (idx) => self . next = Some (idx) , Link :: Entry (_) => self . next = None , } return Some ((None , extra . value)) ; } let idx = self . idx ; if idx == self . len { return None ; } self . idx += 1 ; unsafe { let entry = & (* self . entries) [idx] ; let key = ptr :: read (& entry . key as * const _) ; let value = ptr :: read (& entry . value as * const _) ; self . next = entry . links . map (| l | l . next) ; Some ((Some (key) , value)) } } fn size_hint (& self) -> (usize , Option < usize >) { let lower = self . len - self . idx ; let upper = unsafe { (* self . extra_values) . len () } + lower ; (lower , Some (upper)) } }
};
}
