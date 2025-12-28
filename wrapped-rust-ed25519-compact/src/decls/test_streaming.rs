macro_rules! deps {
    () => {
        Noise!();
        KeyPair!();
    };
}

macro_rules! test_streaming {
    () => {
        deps!();
        # [cfg (feature = "random")] # [test] fn test_streaming () { let kp = KeyPair :: generate () ; let msg1 = "mes" ; let msg2 = "sage" ; let mut st = kp . sk . sign_incremental (Noise :: default ()) ; st . absorb (msg1) ; st . absorb (msg2) ; let signature = st . sign () ; let msg1 = "mess" ; let msg2 = "age" ; let mut st = kp . pk . verify_incremental (& signature) . unwrap () ; st . absorb (msg1) ; st . absorb (msg2) ; assert ! (st . verify () . is_ok ()) ; }
    };
}

test_streaming!();