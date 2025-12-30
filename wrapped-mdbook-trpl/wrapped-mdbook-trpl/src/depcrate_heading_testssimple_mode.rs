// Generated macro for simple_mode (module)
macro_rules! Depcrate_heading_testssimple_mode {
() => {
// Module: crate::heading::tests
// Provides: {"simple_mode"}
// Dependencies: {}
mod simple_mode { use super :: * ; # [test] fn strips_em () { let result = rewrite_headings ("# This is *emphasized* and _this is too_
## Here is *another* and _emphasis style_
### Third *level* _heading_ here
#### Fourth *heading* _level_ text
##### Fifth *level* _heading_ now
###### Last *heading* _level_ test
" , Mode :: Simple ,) ; let expected = rewrite_headings ("# This is emphasized and this is too
## Here is another and emphasis style
### Third level heading here
#### Fourth heading level text
##### Fifth level heading now
###### Last heading level test
" , Mode :: Simple ,) ; assert_eq ! (result . unwrap () , expected . unwrap ()) ; } # [test] fn strips_nested_em () { let result = rewrite_headings ("# *This _is *extra* emphatic_ emphasis*." , Mode :: Simple ,) ; let expected = "# This is extra emphatic emphasis." ; assert_eq ! (result . unwrap () , expected) ; } # [test] fn strips_strong () { let result = rewrite_headings ("# This is **strong** and __this is too__
## Here is **another** and __strong style__
### Third **level** __heading__ here
#### Fourth **heading** __level__ text
##### Fifth **level** __heading__ now
###### Last **heading** __level__ test
" , Mode :: Simple ,) ; let expected = rewrite_headings ("# This is strong and this is too
## Here is another and strong style
### Third level heading here
#### Fourth heading level text
##### Fifth level heading now
###### Last heading level test
" , Mode :: Simple ,) ; assert_eq ! (result . unwrap () , expected . unwrap ()) ; } # [test] fn strips_nested_strong () { let result = rewrite_headings ("# **This __is **extra** emphatic__ emphasis**." , Mode :: Simple ,) ; let expected = "# This is extra emphatic emphasis." ; assert_eq ! (result . unwrap () , expected) ; } # [test] fn strips_code () { let result = rewrite_headings ("# This is `code`
## Here is `another`
### Third `level`
#### Fourth `heading`
##### Fifth `level`
###### Last `heading`
" , Mode :: Simple ,) ; let expected = rewrite_headings ("# This is code
## Here is another
### Third level
#### Fourth heading
##### Fifth level
###### Last heading
" , Mode :: Simple ,) ; assert_eq ! (result . unwrap () , expected . unwrap ()) ; } # [test] fn strips_html () { let result = rewrite_headings ("# This is <span>html</span>
## Here is <span>another</span>
### Third <span>level</span>
#### Fourth <span>heading</span>
##### Fifth <span>level</span>
###### Last <span>heading</span>
" , Mode :: Simple ,) ; let expected = rewrite_headings ("# This is html
## Here is another
### Third level
#### Fourth heading
##### Fifth level
###### Last heading
" , Mode :: Simple ,) ; assert_eq ! (result . unwrap () , expected . unwrap ()) ; } # [test] fn strips_strikethrough () { let result = rewrite_headings ("# This is ~~strikethrough~~
## Here is ~~another~~
### Third ~~level~~
#### Fourth ~~heading~~
##### Fifth ~~level~~
###### Last ~~heading~~
" , Mode :: Simple ,) ; let expected = rewrite_headings ("# This is strikethrough
## Here is another
### Third level
#### Fourth heading
##### Fifth level
###### Last heading
" , Mode :: Simple ,) ; assert_eq ! (result . unwrap () , expected . unwrap ()) ; } # [test] fn strips_nested_combinations () { let result = rewrite_headings ("# **Nested ~~strikethrough _emphasis_ fun~~ times**" , Mode :: Simple ,) ; let expected = rewrite_headings ("# Nested strikethrough emphasis fun times" , Mode :: Simple ,) ; assert_eq ! (result . unwrap () , expected . unwrap ()) } }
};
}
