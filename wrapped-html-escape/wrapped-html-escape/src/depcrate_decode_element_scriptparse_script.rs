// Generated macro for parse_script (macro)
macro_rules! Depcrate_decode_element_scriptparse_script {
() => {
// Module: crate::decode::element::script
// Provides: {"parse_script"}
// Dependencies: {}
macro_rules ! parse_script { ($ e : expr , $ step : ident , $ b : block , $ bq : block , $ bc : block $ (, $ ($ addi : expr) ,+) ?) => { match $ step { 0 => { match $ e { b'<' => $ step = 1 , b'\\' => $ step = 100 , _ => () , } } 1 => { match $ e { b'\\' => $ step = 2 , _ => () , } } 2 => { match $ e { b'/' => $ step = 3 , b'!' => $ step = 10 , $ ($ (| $ addi) + => { $ step = 0 ; $ bq } ,) ? _ => $ step = 0 , } } 3 => { match $ e { b's' | b'S' => $ step = 4 , b'\\' => $ step = 100 , _ => $ step = 0 , } } 4 => { match $ e { b'c' | b'C' => $ step = 5 , b'\\' => $ step = 100 , _ => $ step = 0 , } } 5 => { match $ e { b'r' | b'R' => $ step = 6 , b'\\' => $ step = 100 , _ => $ step = 0 , } } 6 => { match $ e { b'i' | b'I' => $ step = 7 , b'\\' => $ step = 100 , _ => $ step = 0 , } } 7 => { match $ e { b'p' | b'P' => $ step = 8 , b'\\' => $ step = 100 , _ => $ step = 0 , } } 8 => { match $ e { b't' | b'T' => $ step = 9 , b'\\' => $ step = 100 , _ => $ step = 0 , } } 9 => { match $ e { b'>' | 9 ..= 13 | 28 ..= 32 => { $ step = 0 ; $ b } , b'\\' => $ step = 100 , _ => $ step = 0 , } } 10 => { match $ e { b'-' => $ step = 11 , b'\\' => $ step = 100 , _ => $ step = 0 , } } 11 => { match $ e { b'-' => { $ step = 0 ; $ bc } , b'\\' => $ step = 100 , _ => $ step = 0 , } } 100 => { match $ e { b'<' => $ step = 1 , $ ($ (| $ addi) + => { $ step = 0 ; $ bq } ,) ? _ => $ step = 0 , } } _ => unreachable ! () , } } ; }
};
}
