// Generated macro for try_bind (function)
macro_rules! Depcratetry_bind {
() => {
// Module: crate
// Provides: {"try_bind"}
// Dependencies: {}
fn try_bind (input : TokenStream) -> Result < TokenStream > { let (stmt , literal) = { let mut iter = input . into_iter () ; let stmt = iter . next () . unwrap () ; let literal = iter . next () . unwrap () ; assert ! (iter . next () . is_none ()) ; (stmt , literal) } ; let Some (literal) = into_literal (& literal) else { return Err ("expected a plain string literal" . to_string ()) ; } ; let call_site = literal . span () ; let string_lit = match StringLit :: try_from (literal) { Ok (string_lit) => string_lit , Err (e) => return Ok (e . to_compile_error ()) , } ; let sql = string_lit . value () ; let mut parser = Parser :: new (sql . as_bytes ()) ; let ast = match parser . next () { Ok (None) => return Err ("Invalid input" . to_owned ()) , Err (err) => { return Err (err . to_string ()) ; } Ok (Some (ast)) => ast , } ; let mut info = ParameterInfo :: default () ; if let Err (err) = ast . to_tokens (& mut info) { return Err (err . to_string ()) ; } if info . count == 0 { return Ok (TokenStream :: new ()) ; } if info . count as usize != info . names . len () { return Err ("Mixing named and numbered parameters is not supported." . to_string ()) ; } let mut res = TokenStream :: new () ; for (i , name) in info . names . iter () . enumerate () { res . extend (Some (stmt . clone ())) ; res . extend (respan (parse_ts (& format ! (".raw_bind_parameter({}, &{})?;" , i + 1 , & name [1 ..])) , call_site ,)) ; } Ok (res) }
};
}
