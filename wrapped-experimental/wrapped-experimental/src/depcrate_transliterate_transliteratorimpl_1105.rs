// Generated macro for impl_1105 (impl)
macro_rules! Depcrate_transliterate_transliteratorimpl_1105 {
() => {
// Module: crate::transliterate::transliterator
// Provides: {"impl_1105"}
// Dependencies: {}
impl < 'a > VarTableElement < 'a > { fn into_replacer (self) -> Option < SpecialReplacer < 'a > > { Some (match self { Self :: Compound (elt) => SpecialReplacer :: Compound (elt) , Self :: FunctionCall (elt) => SpecialReplacer :: FunctionCall (elt) , Self :: BackReference (elt) => SpecialReplacer :: BackReference (elt) , Self :: LeftPlaceholderCursor (elt) => SpecialReplacer :: LeftPlaceholderCursor (elt) , Self :: RightPlaceholderCursor (elt) => SpecialReplacer :: RightPlaceholderCursor (elt) , Self :: PureCursor => SpecialReplacer :: PureCursor , _ => return None , }) } fn into_matcher (self) -> Option < SpecialMatcher < 'a > > { Some (match self { Self :: Compound (elt) => SpecialMatcher :: Compound (elt) , Self :: Quantifier (kind , elt) => SpecialMatcher :: Quantifier (kind , elt) , Self :: Segment (elt) => SpecialMatcher :: Segment (elt) , Self :: UnicodeSet (elt) => SpecialMatcher :: UnicodeSet (elt) , Self :: AnchorEnd => SpecialMatcher :: AnchorEnd , Self :: AnchorStart => SpecialMatcher :: AnchorStart , _ => return None , }) } }
};
}
