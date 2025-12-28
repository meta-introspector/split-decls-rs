macro_rules! deps {
    () => {
        IndexPackage!();
    };
}

macro_rules! escaped_char_in_index_json_blob {
    () => {
        deps!();
        # [test] fn escaped_char_in_index_json_blob () { let _ : IndexPackage < '_ > = serde_json :: from_str (r#"{"name":"a","vers":"0.0.1","deps":[],"cksum":"bae3","features":{}}"# ,) . unwrap () ; let _ : IndexPackage < '_ > = serde_json :: from_str (r#"{"name":"a","vers":"0.0.1","deps":[],"cksum":"bae3","features":{"test":["k","q"]},"links":"a-sys"}"#) . unwrap () ; let _ : IndexPackage < '_ > = serde_json :: from_str (r#"{
        "name":"This name has a escaped cher in it \n\t\" ",
        "vers":"0.0.1",
        "deps":[{
            "name": " \n\t\" ",
            "req": " \n\t\" ",
            "features": [" \n\t\" "],
            "optional": true,
            "default_features": true,
            "target": " \n\t\" ",
            "kind": " \n\t\" ",
            "registry": " \n\t\" "
        }],
        "cksum":"bae3",
        "features":{"test \n\t\" ":["k \n\t\" ","q \n\t\" "]},
        "links":" \n\t\" "}"# ,) . unwrap () ; }
    };
}

escaped_char_in_index_json_blob!()