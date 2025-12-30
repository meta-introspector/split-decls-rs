// Generated macro for rewrite (function)
macro_rules! Depcrate_noterewrite {
() => {
// Module: crate::note
// Provides: {"rewrite"}
// Dependencies: {}
pub fn rewrite (text : & str) -> String { let parser = crate :: parser (text) ; let mut events = Vec :: new () ; let mut state = Default ; for event in parser { match (& mut state , event) { (Default , Start (Tag :: BlockQuote (_))) => { state = StartingBlockquote (vec ! [Start (Tag :: BlockQuote (None))]) ; } (StartingBlockquote (blockquote_events) , Text (content)) => { if content . starts_with ("Note: ") { events . extend ([SoftBreak , SoftBreak , Html (r#"<section class="note" aria-role="note">"# . into () ,) , SoftBreak , SoftBreak , Start (Tag :: Paragraph) , Text (content) ,]) ; state = InNote ; } else { events . append (blockquote_events) ; events . push (Text (content)) ; state = Default ; } } (StartingBlockquote (_blockquote_events) , heading @ Start (Tag :: Heading { .. }) ,) => { events . extend ([SoftBreak , SoftBreak , Html (r#"<section class="note" aria-role="note">"# . into ()) , SoftBreak , SoftBreak , heading ,]) ; state = InNote ; } (StartingBlockquote (ref mut events) , Start (tag)) => { events . push (Start (tag)) ; } (InNote , End (TagEnd :: BlockQuote (_))) => { events . extend ([SoftBreak , SoftBreak , Html ("</section>" . into ()) ,]) ; state = Default ; } (_ , event) => { events . push (event) ; } } } let mut buf = String :: new () ; cmark (events . into_iter () , & mut buf) . unwrap () ; buf }
};
}
