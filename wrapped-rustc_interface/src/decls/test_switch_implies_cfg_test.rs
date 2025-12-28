macro_rules! test_switch_implies_cfg_test {
    () => {
        # [test] fn test_switch_implies_cfg_test () { sess_and_cfg (& ["--test"] , | _sess , cfg | { assert ! (cfg . contains (& (sym :: test , None))) ; }) }
    };
}

test_switch_implies_cfg_test!()