macro_rules! deps {
    () => {
        Expr!();
        Pat!();
        Walkable!();
        AttrVec!();
    };
}

macro_rules! Arm {
    () => {
        deps!();
        # [doc = " An arm of a 'match'."] # [doc = ""] # [doc = " E.g., `0..=10 => { println!(\"match!\") }` as in"] # [doc = ""] # [doc = " ```"] # [doc = " match 123 {"] # [doc = "     0..=10 => { println!(\"match!\") },"] # [doc = "     _ => { println!(\"no match!\") },"] # [doc = " }"] # [doc = " ```"] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct Arm { pub attrs : AttrVec , # [doc = " Match arm pattern, e.g. `10` in `match foo { 10 => {}, _ => {} }`."] pub pat : Box < Pat > , # [doc = " Match arm guard, e.g. `n > 10` in `match foo { n if n > 10 => {}, _ => {} }`."] pub guard : Option < Box < Expr > > , # [doc = " Match arm body. Omitted if the pattern is a never pattern."] pub body : Option < Box < Expr > > , pub span : Span , pub id : NodeId , pub is_placeholder : bool , }
    };
}

Arm!();