// Generated macro for rewrite_headings (function)
macro_rules! Depcrate_headingrewrite_headings {
() => {
// Module: crate::heading
// Provides: {"rewrite_headings"}
// Dependencies: {}
fn rewrite_headings (src : & str , mode : Mode) -> anyhow :: Result < String > { if mode == Mode :: Default { return Ok (src . into ()) ; } # [derive (Default)] struct State < 'e > { in_heading : bool , events : Vec < Event < 'e > > , } let final_state : State = crate :: parser (src) . try_fold (State :: default () , | mut state , event | -> anyhow :: Result < State > { if state . in_heading { match event { Event :: Start (Tag :: Emphasis | Tag :: Strong | Tag :: Strikethrough ,) | Event :: End (TagEnd :: Emphasis | TagEnd :: Strong | TagEnd :: Strikethrough ,) | Event :: InlineHtml (_) => { } Event :: Code (code) => { state . events . push (Event :: Text (code)) ; } Event :: End (TagEnd :: Heading (_)) => { state . in_heading = false ; state . events . push (event) ; } _ => state . events . push (event) , } } else if matches ! (event , Event :: Start (Tag :: Heading { .. })) { state . events . push (event) ; state . in_heading = true ; } else { state . events . push (event) ; } Ok (state) } ,) ? ; if final_state . in_heading { return Err (anyhow ! ("Unclosed heading")) ; } let mut rewritten = String :: new () ; cmark (final_state . events . into_iter () , & mut rewritten) ? ; Ok (rewritten) }
};
}
