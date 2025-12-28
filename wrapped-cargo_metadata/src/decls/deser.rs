macro_rules! deps {
    () => {
        SuiteEvent!();
        TestEvent!();
        TestMessage!();
    };
}

macro_rules! deser {
    () => {
        deps!();
        # [test] fn deser () { macro_rules ! run { ($ ($ input : literal parses to $ output : expr) ,+) => { $ (assert_eq ! (dbg ! (serde_json :: from_str ::< TestMessage > ($ input)) . unwrap () , $ output) ;) + } ; } run ! [r#"{ "type": "suite", "event": "started", "test_count": 2 }"# parses to TestMessage :: Suite (SuiteEvent :: Started { test_count : 2 }) , r#"{ "type": "test", "event": "started", "name": "fail" }"# parses to TestMessage :: Test (TestEvent :: Started { name : "fail" . into () }) , r#"{ "type": "test", "name": "fail", "event": "ok", "exec_time": 0.000003428, "stdout": "hello world" }"# parses to TestMessage :: Test (TestEvent :: Ok { name : "fail" . into () , exec_time : 0.000003428 , stdout : Some ("hello world" . into ()) }) , r#"{ "type": "test", "event": "started", "name": "nope" }"# parses to TestMessage :: Test (TestEvent :: Started { name : "nope" . into () }) , r#"{ "type": "test", "name": "nope", "event": "ignored" }"# parses to TestMessage :: Test (TestEvent :: Ignored { name : "nope" . into () }) , r#"{ "type": "suite", "event": "ok", "passed": 1, "failed": 0, "ignored": 1, "measured": 0, "filtered_out": 0, "exec_time": 0.000684028 }"# parses to TestMessage :: Suite (SuiteEvent :: Ok { passed : 1 , failed : 0 , ignored : 1 , measured : 0 , filtered_out : 0 , exec_time : 0.000684028 })] ; run ! [r#"{ "type": "suite", "event": "started", "test_count": 2 }"# parses to TestMessage :: Suite (SuiteEvent :: Started { test_count : 2 }) , r#"{ "type": "test", "event": "started", "name": "fail" }"# parses to TestMessage :: Test (TestEvent :: Started { name : "fail" . into () }) , r#"{ "type": "test", "event": "started", "name": "benc" }"# parses to TestMessage :: Test (TestEvent :: Started { name : "benc" . into () }) , r#"{ "type": "bench", "name": "benc", "median": 0, "deviation": 0 }"# parses to TestMessage :: Bench { name : "benc" . into () , median : 0. , deviation : 0. , mib_per_second : None } , r#"{ "type": "test", "name": "fail", "event": "failed", "exec_time": 0.000081092, "stdout": "thread 'fail' panicked" }"# parses to TestMessage :: Test (TestEvent :: Failed { name : "fail" . into () , exec_time : 0.000081092 , stdout : Some ("thread 'fail' panicked" . into ()) , reason : None , message : None }) , r#"{ "type": "suite", "event": "failed", "passed": 0, "failed": 1, "ignored": 0, "measured": 1, "filtered_out": 0, "exec_time": 0.000731068 }"# parses to TestMessage :: Suite (SuiteEvent :: Failed { passed : 0 , failed : 1 , ignored : 0 , measured : 1 , filtered_out : 0 , exec_time : 0.000731068 })] ; }
    };
}

deser!()