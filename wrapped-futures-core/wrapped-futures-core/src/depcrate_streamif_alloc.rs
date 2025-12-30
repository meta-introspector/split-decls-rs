// Generated macro for if_alloc (module)
macro_rules! Depcrate_streamif_alloc {
() => {
// Module: crate::stream
// Provides: {"if_alloc"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod if_alloc { use super :: * ; use alloc :: boxed :: Box ; impl < S : ? Sized + Stream + Unpin > Stream for Box < S > { type Item = S :: Item ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Pin :: new (& mut * * self) . poll_next (cx) } fn size_hint (& self) -> (usize , Option < usize >) { (* * self) . size_hint () } } # [cfg (feature = "std")] impl < S : Stream > Stream for std :: panic :: AssertUnwindSafe < S > { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < S :: Item > > { unsafe { self . map_unchecked_mut (| x | & mut x . 0) } . poll_next (cx) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } } impl < S : ? Sized + FusedStream + Unpin > FusedStream for Box < S > { fn is_terminated (& self) -> bool { < S as FusedStream > :: is_terminated (& * * self) } } }
};
}
