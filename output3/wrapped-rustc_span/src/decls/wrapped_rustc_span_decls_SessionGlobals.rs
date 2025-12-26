use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Per-session global variables: this struct is stored in thread-local storage
/// in such a way that it is accessible without any kind of handle to all
/// threads within the compilation session, but is not accessible outside the
/// session.
pub struct SessionGlobals {
    symbol_interner: symbol::Interner,
    span_interner: Lock<span_encoding::SpanInterner>,
    /// Maps a macro argument token into use of the corresponding metavariable in the macro body.
    /// Collisions are possible and processed in `maybe_use_metavar_location` on best effort basis.
    metavar_spans: MetavarSpansMap,
    hygiene_data: Lock<hygiene::HygieneData>,
    /// The session's source map, if there is one. This field should only be
    /// used in places where the `Session` is truly not available, such as
    /// `<Span as Debug>::fmt`.
    source_map: Option<Arc<SourceMap>>,
}
