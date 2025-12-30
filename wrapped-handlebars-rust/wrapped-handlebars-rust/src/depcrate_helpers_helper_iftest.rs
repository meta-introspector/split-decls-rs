// Generated macro for test (module)
macro_rules! Depcrate_helpers_helper_iftest {
() => {
// Module: crate::helpers::helper_if
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: helpers :: WITH_HELPER ; use crate :: registry :: Registry ; use serde_json :: value :: Value as Json ; use std :: str :: FromStr ; # [test] fn test_if () { let mut handlebars = Registry :: new () ; assert ! (handlebars . register_template_string ("t0" , "{{#if this}}hello{{/if}}") . is_ok ()) ; assert ! (handlebars . register_template_string ("t1" , "{{#unless this}}hello{{else}}world{{/unless}}") . is_ok ()) ; let r0 = handlebars . render ("t0" , & true) ; assert_eq ! (r0 . ok () . unwrap () , "hello" . to_string ()) ; let r1 = handlebars . render ("t1" , & true) ; assert_eq ! (r1 . ok () . unwrap () , "world" . to_string ()) ; let r2 = handlebars . render ("t0" , & false) ; assert_eq ! (r2 . ok () . unwrap () , String :: new ()) ; } # [test] fn test_if_context () { let json_str = r#"{"a":{"b":99,"c":{"d": true}}}"# ; let data = Json :: from_str (json_str) . unwrap () ; let mut handlebars = Registry :: new () ; handlebars . register_helper ("with" , Box :: new (WITH_HELPER)) ; assert ! (handlebars . register_template_string ("t0" , "{{#if a.c.d}}hello {{a.b}}{{/if}}") . is_ok ()) ; assert ! (handlebars . register_template_string ("t1" , "{{#with a}}{{#if c.d}}hello {{../a.b}}{{/if}}{{/with}}") . is_ok ()) ; let r0 = handlebars . render ("t0" , & data) ; assert_eq ! (r0 . unwrap () , "hello 99" . to_string ()) ; let r1 = handlebars . render ("t1" , & data) ; assert_eq ! (r1 . unwrap () , "hello 99" . to_string ()) ; } # [test] fn test_if_else_chain () { let handlebars = Registry :: new () ; assert_eq ! ("0" . to_owned () , handlebars . render_template ("{{#if a}}1{{else if b}}2{{else}}0{{/if}}" , & json ! ({ "d" : 1 })) . unwrap ()) ; } # [test] fn test_if_else_chain2 () { let handlebars = Registry :: new () ; assert_eq ! ("3" . to_owned () , handlebars . render_template ("{{#if a}}1{{else if b}}2{{else if c}}3{{else if d}}4{{else}}0{{/if}}" , & json ! ({ "c" : 1 , "d" : 1 })) . unwrap ()) ; } # [test] fn test_if_else_chain3 () { let handlebars = Registry :: new () ; assert_eq ! ("4" . to_owned () , handlebars . render_template ("{{#if a}}1{{else if b}}2{{else if c}}3{{else if d}}4{{/if}}" , & json ! ({ "d" : 1 })) . unwrap ()) ; } # [test] fn test_if_else_chain4 () { let handlebars = Registry :: new () ; assert_eq ! ("1" . to_owned () , handlebars . render_template ("{{#if a}}1{{else if b}}2{{else if c}}3{{else if d}}4{{/if}}" , & json ! ({ "a" : 1 })) . unwrap ()) ; } # [test] fn test_if_include_zero () { use std :: f64 ; let handlebars = Registry :: new () ; assert_eq ! ("0" . to_owned () , handlebars . render_template ("{{#if a}}1{{else}}0{{/if}}" , & json ! ({ "a" : 0 })) . unwrap ()) ; assert_eq ! ("1" . to_owned () , handlebars . render_template ("{{#if a includeZero=true}}1{{else}}0{{/if}}" , & json ! ({ "a" : 0 })) . unwrap ()) ; assert_eq ! ("0" . to_owned () , handlebars . render_template ("{{#if a includeZero=true}}1{{else}}0{{/if}}" , & json ! ({ "a" : f64 :: NAN })) . unwrap ()) ; } # [test] fn test_invisible_line_stripping () { let hbs = Registry :: new () ; assert_eq ! ("yes\n" , hbs . render_template ("{{#if a}}\nyes\n{{/if}}\n" , & json ! ({ "a" : true })) . unwrap ()) ; assert_eq ! ("yes\r\n" , hbs . render_template ("{{#if a}}\r\nyes\r\n{{/if}}\r\n" , & json ! ({ "a" : true })) . unwrap ()) ; assert_eq ! ("x\ny" , hbs . render_template ("{{#if a}}x{{/if}}\ny" , & json ! ({ "a" : true })) . unwrap ()) ; assert_eq ! ("y\nz" , hbs . render_template ("{{#if a}}\nx\n{{^}}\ny\n{{/if}}\nz" , & json ! ({ "a" : false })) . unwrap ()) ; assert_eq ! (r"yes
  foo
  bar
  baz" , hbs . render_template (r"yes
  {{#if true}}
  foo
  bar
  {{/if}}
  baz" , & json ! ({ })) . unwrap ()) ; assert_eq ! (r"  foo
  bar
  baz" , hbs . render_template (r"  {{#if true}}
  foo
  bar
  {{/if}}
  baz" , & json ! ({ })) . unwrap ()) ; } }
};
}
