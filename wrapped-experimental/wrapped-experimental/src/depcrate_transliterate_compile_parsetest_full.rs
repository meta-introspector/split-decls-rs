// Generated macro for test_full (function)
macro_rules! Depcrate_transliterate_compile_parsetest_full {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"test_full"}
// Dependencies: {}
# [test] fn test_full () { let source = r"

    # these are skipped:
    use variable range 0x70 0x72 ;
    use variable range 0x1 0x2 ;

    :: [a-z\]] ; :: [b-z] Latin/BGN ;
    :: Source-Target/Variant () ;::([b-z]Target-Source/Variant) ;
    :: [a-z] Any ([b-z] Target-Source/Variant);

    $my_var = an arbitrary section ',' some quantifiers *+? 'and other variables: $var' $var  ;
    $innerMinus = '-' ;
    $minus = $innerMinus ;
    $good_set = [a $minus z] ;

    ^ (start) { key ' key '+ $good_set } > $102 }  post\-context$;
    # contexts are optional
    target < source [{set\ with\ string}];
    # contexts can be empty
    { 'source-or-target' } <> { 'target-or-source' } ;

    (nested (sections)+ are () so fun) > ;

    . > ;

    :: ([inverse-filter]) ;
    " ; parse (source) . map_err (| e | e . explain (source)) . unwrap () ; }
};
}
