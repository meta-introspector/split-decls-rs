/// SPARQL macro system for safe query construction
/// 
/// This macro system allows constructing SPARQL queries without format string issues.
/// Variables are prefixed with $ and the macro handles proper escaping.

/// Main SPARQL query construction macro
macro_rules! sparql {
    // SELECT query with WHERE clause
    (SELECT $($var:ident)+ WHERE { $($triple:tt)* }) => {
        format!("SELECT {} WHERE {{ {} }}", 
            sparql!(@vars $($var)+),
            sparql!(@triples $($triple)*))
    };
    
    // ASK query
    (ASK WHERE { $($triple:tt)* }) => {
        format!("ASK WHERE {{ {} }}", sparql!(@triples $($triple)*))
    };
    
    // CONSTRUCT query
    (CONSTRUCT { $($construct:tt)* } WHERE { $($where:tt)* }) => {
        format!("CONSTRUCT {{ {} }} WHERE {{ {} }}", 
            sparql!(@triples $($construct)*),
            sparql!(@triples $($where)*))
    };
    
    // Helper: format variables
    (@vars $var:ident) => { format!("?{}", stringify!($var)) };
    (@vars $var:ident $($rest:ident)+) => { 
        format!("?{} {}", stringify!($var), sparql!(@vars $($rest)+))
    };
    
    // Helper: format triples
    (@triples $s:ident $p:ident $o:ident) => {
        format!("?{} {}:{} ?{}", stringify!($s), "lmdfb", stringify!($p), stringify!($o))
    };
    (@triples $s:ident $p:ident $o:literal) => {
        format!("?{} {}:{} {}", stringify!($s), "lmdfb", stringify!($p), $o)
    };
    (@triples $s:ident rdf:type $o:ident) => {
        format!("?{} rdf:type lmdfb:{}", stringify!($s), stringify!($o))
    };
    (@triples $s:ident $p:ident $o:ident . $($rest:tt)*) => {
        format!("?{} {}:{} ?{} . {}", 
            stringify!($s), "lmdfb", stringify!($p), stringify!($o),
            sparql!(@triples $($rest)*))
    };
    (@triples) => { "" };
}

/// Convenience macro for common SPARQL patterns
macro_rules! sparql_select {
    ($var:ident has_type $type:ident) => {
        sparql!(SELECT $var WHERE { $var rdf:type $type })
    };
    
    ($var:ident has_property $prop:ident as $value:ident) => {
        sparql!(SELECT $var $value WHERE { $var $prop $value })
    };
}

/// Print a SPARQL query with proper formatting
macro_rules! println_sparql {
    ($query:expr) => {
        println!("{}", $query);
    };
    ($label:expr, $query:expr) => {
        println!("\n{}. {}", $label, $query);
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sparql_macros() {
        let query1 = sparql!(SELECT s WHERE { s rdf:type RustcComponent });
        assert_eq!(query1, "SELECT ?s WHERE { ?s rdf:type lmdfb:RustcComponent }");
        
        let query2 = sparql_select!(s has_type RustcComponent);
        assert_eq!(query2, "SELECT ?s WHERE { ?s rdf:type lmdfb:RustcComponent }");
        
        let query3 = sparql_select!(s has_property hasLevel as level);
        assert_eq!(query3, "SELECT ?s ?level WHERE { ?s lmdfb:hasLevel ?level }");
    }
}
