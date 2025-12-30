// Generated macro for test_yaml_serialization (function)
macro_rules! Depcrate_serializationtest_yaml_serialization {
() => {
// Module: crate::serialization
// Provides: {"test_yaml_serialization"}
// Dependencies: {}
# [test] fn test_yaml_serialization () { let yaml = serialize_content (Content :: Map (vec ! [(Content :: from ("env") , Content :: Seq (vec ! [Content :: from ("ENVIRONMENT") , Content :: from ("production") ,]) ,) , (Content :: from ("cmdline") , Content :: Seq (vec ! [Content :: from ("my-tool") , Content :: from ("run")]) ,) ,]) , SerializationFormat :: Yaml ,) ; crate :: assert_snapshot ! (& yaml , @ r"
    env:
      - ENVIRONMENT
      - production
    cmdline:
      - my-tool
      - run
    ") ; let inline_yaml = serialize_content (Content :: Map (vec ! [(Content :: from ("env") , Content :: Seq (vec ! [Content :: from ("ENVIRONMENT") , Content :: from ("production") ,]) ,) , (Content :: from ("cmdline") , Content :: Seq (vec ! [Content :: from ("my-tool") , Content :: from ("run")]) ,) ,]) , SerializationFormat :: Yaml ,) ; crate :: assert_snapshot ! (& inline_yaml , @ r"
    env:
      - ENVIRONMENT
      - production
    cmdline:
      - my-tool
      - run
    ") ; }
};
}
