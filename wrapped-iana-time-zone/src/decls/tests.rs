macro_rules! tests {
    () => {
        # [cfg (test)] mod tests { use super :: * ; # [test] fn get_current () { println ! ("current: {}" , get_timezone () . unwrap ()) ; } }
    };
}

tests!()