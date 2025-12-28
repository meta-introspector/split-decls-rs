macro_rules! deps {
    () => {
        DocComment!();
    };
}

macro_rules! consume {
    () => {
        deps!();
        # [doc = " Consume pairs to matches `Rule::grammar_doc`, `Rule::line_doc` into `DocComment`"] # [doc = ""] # [doc = " e.g."] # [doc = ""] # [doc = " a pest file:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " //! This is a grammar doc"] # [doc = " /// line doc 1"] # [doc = " /// line doc 2"] # [doc = " foo = {}"] # [doc = ""] # [doc = " /// line doc 3"] # [doc = " bar = {}"] # [doc = " ```"] # [doc = ""] # [doc = " Then will get:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " grammar_doc = \"This is a grammar doc\""] # [doc = " line_docs = { \"foo\": \"line doc 1\\nline doc 2\", \"bar\": \"line doc 3\" }"] # [doc = " ```"] pub fn consume (pairs : Pairs < '_ , Rule >) -> DocComment { let mut grammar_doc = String :: new () ; let mut line_docs : HashMap < String , String > = HashMap :: new () ; let mut line_doc = String :: new () ; for pair in pairs { match pair . as_rule () { Rule :: grammar_doc => { let inner_doc = pair . into_inner () . next () . unwrap () ; grammar_doc . push_str (inner_doc . as_str ()) ; grammar_doc . push ('\n') ; } Rule :: grammar_rule => { if let Some (inner) = pair . into_inner () . next () { match inner . as_rule () { Rule :: line_doc => { if let Some (inner_doc) = inner . into_inner () . next () { line_doc . push_str (inner_doc . as_str ()) ; line_doc . push ('\n') ; } } Rule :: identifier => { if ! line_doc . is_empty () { let rule_name = inner . as_str () . to_owned () ; line_doc . pop () ; line_docs . insert (rule_name , line_doc . clone ()) ; line_doc . clear () ; } } _ => () , } } } _ => () , } } if ! grammar_doc . is_empty () { grammar_doc . pop () ; } DocComment { grammar_doc , line_docs , } }
    };
}

consume!()