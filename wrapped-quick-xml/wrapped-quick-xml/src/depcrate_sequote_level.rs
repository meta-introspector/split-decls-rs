// Generated macro for quote_level (module)
macro_rules! Depcrate_sequote_level {
() => {
// Module: crate::se
// Provides: {"quote_level"}
// Dependencies: {}
# [cfg (test)] mod quote_level { use super :: * ; use pretty_assertions :: assert_eq ; use serde :: Serialize ; # [derive (Debug , PartialEq , Serialize)] struct Element (& 'static str) ; # [derive (Debug , PartialEq , Serialize)] struct Example { # [serde (rename = "@attribute")] attribute : & 'static str , element : Element , } # [test] fn default_ () { let example = Example { attribute : "special chars: &, <, >, \", '" , element : Element ("special chars: &, <, >, \", '") , } ; let mut buffer = String :: new () ; let ser = Serializer :: new (& mut buffer) ; example . serialize (ser) . unwrap () ; assert_eq ! (buffer , "<Example attribute=\"special chars: &amp;, &lt;, &gt;, &quot;, '\">\
                <element>special chars: &amp;, &lt;, &gt;, \", '</element>\
            </Example>") ; } # [test] fn minimal () { let example = Example { attribute : "special chars: &, <, >, \", '" , element : Element ("special chars: &, <, >, \", '") , } ; let mut buffer = String :: new () ; let mut ser = Serializer :: new (& mut buffer) ; ser . set_quote_level (QuoteLevel :: Minimal) ; example . serialize (ser) . unwrap () ; assert_eq ! (buffer , "<Example attribute=\"special chars: &amp;, &lt;, >, &quot;, '\">\
                <element>special chars: &amp;, &lt;, >, \", '</element>\
            </Example>") ; } # [test] fn partial () { let example = Example { attribute : "special chars: &, <, >, \", '" , element : Element ("special chars: &, <, >, \", '") , } ; let mut buffer = String :: new () ; let mut ser = Serializer :: new (& mut buffer) ; ser . set_quote_level (QuoteLevel :: Partial) ; example . serialize (ser) . unwrap () ; assert_eq ! (buffer , "<Example attribute=\"special chars: &amp;, &lt;, &gt;, &quot;, '\">\
                <element>special chars: &amp;, &lt;, &gt;, \", '</element>\
            </Example>") ; } # [test] fn full () { let example = Example { attribute : "special chars: &, <, >, \", '" , element : Element ("special chars: &, <, >, \", '") , } ; let mut buffer = String :: new () ; let mut ser = Serializer :: new (& mut buffer) ; ser . set_quote_level (QuoteLevel :: Full) ; example . serialize (ser) . unwrap () ; assert_eq ! (buffer , "<Example attribute=\"special chars: &amp;, &lt;, &gt;, &quot;, &apos;\">\
                <element>special chars: &amp;, &lt;, &gt;, &quot;, &apos;</element>\
            </Example>") ; } }
};
}
