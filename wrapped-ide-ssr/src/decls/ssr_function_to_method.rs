macro_rules! ssr_function_to_method {
    () => {
        # [test] fn ssr_function_to_method () { assert_ssr_transform ("my_function($a, $b) ==>> ($a).my_method($b)" , "fn my_function() {} fn main() { loop { my_function( other_func(x, y), z + w) } }" , expect ! [["fn my_function() {} fn main() { loop { (other_func(x, y)).my_method(z + w) } }"]] ,) }
    };
}

ssr_function_to_method!()