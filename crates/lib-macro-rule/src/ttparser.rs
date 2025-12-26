
#[derive(Clone, Debug)]
pub struct TtParser<'a> {
    pub sess: &'a ParseSess,
    pub token_tree: Cow<'a, TokenStream>,
    pub idx: usize,
    pub dot2_is_not_dotdot: bool,
    // The `Ident` of the macro name for better error messages.
    pub macro_name: Ident,
    pub expansion_seqs: Vec<Box<[MatchedSeq]>>,

    // mbe::macro_parser functions needed for TtParser:
    // parse_tt: needs MacroRule, Tracker, MatcherLoc
    // new: needs Ident
}

impl<'a> TtParser<'a> {
    pub fn new(macro_name: Ident) -> Self {
        unimplemented!()
    }

    pub fn parse_tt<'matcher, T: super::Tracker<'matcher>>(
        &mut self,
        _arg: &mut Cow<'a, Parser<'a>>,
        _matcher: &'matcher [MatcherLoc],
        _track: &mut T,
    ) -> ParseResult<T::Failure> {
        unimplemented!()
    }
}

