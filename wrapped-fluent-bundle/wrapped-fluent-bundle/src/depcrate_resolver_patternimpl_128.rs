// Generated macro for impl_128 (impl)
macro_rules! Depcrate_resolver_patternimpl_128 {
() => {
// Module: crate::resolver::pattern
// Provides: {"impl_128"}
// Dependencies: {}
impl < 'bundle > WriteValue < 'bundle > for ast :: Pattern < & 'bundle str > { fn write < 'ast , 'args , 'errors , W , R , M > (& 'ast self , w : & mut W , scope : & mut Scope < 'bundle , 'ast , 'args , 'errors , R , M > ,) -> fmt :: Result where W : fmt :: Write , R : Borrow < FluentResource > , M : MemoizerKind , { let len = self . elements . len () ; for elem in & self . elements { if scope . dirty { return Ok (()) ; } match elem { ast :: PatternElement :: TextElement { value } => { if let Some (ref transform) = scope . bundle . transform { w . write_str (& transform (value)) ? ; } else { w . write_str (value) ? ; } } ast :: PatternElement :: Placeable { expression } => { scope . placeables += 1 ; if scope . placeables > MAX_PLACEABLES { scope . dirty = true ; scope . add_error (ResolverError :: TooManyPlaceables) ; return Ok (()) ; } let needs_isolation = scope . bundle . use_isolating && len > 1 && ! matches ! (expression , ast :: Expression :: Inline (ast :: InlineExpression :: MessageReference { .. } ,) | ast :: Expression :: Inline (ast :: InlineExpression :: TermReference { .. } ,) | ast :: Expression :: Inline (ast :: InlineExpression :: StringLiteral { .. } ,)) ; if needs_isolation { w . write_char ('\u{2068}') ? ; } scope . maybe_track (w , self , expression) ? ; if needs_isolation { w . write_char ('\u{2069}') ? ; } } } } Ok (()) } fn write_error < W > (& self , _w : & mut W) -> fmt :: Result where W : fmt :: Write , { unreachable ! () } }
};
}
