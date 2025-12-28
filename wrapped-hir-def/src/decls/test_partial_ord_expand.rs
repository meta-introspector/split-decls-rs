macro_rules! test_partial_ord_expand {
    () => {
        # [test] fn test_partial_ord_expand () { check (r#"
//- minicore: derive, ord
#[derive(PartialOrd, Ord)]
enum Command {
    Move { x: i32, y: i32 },
    Do(&'static str),
    Jump,
}
"# , expect ! [[r#"
#[derive(PartialOrd, Ord)]
enum Command {
    Move { x: i32, y: i32 },
    Do(&'static str),
    Jump,
}

impl <> $crate::cmp::PartialOrd for Command< > where {
    fn partial_cmp(&self , other: &Self ) -> $crate::option::Option<$crate::cmp::Ordering> {
        match $crate::intrinsics::discriminant_value(self ).partial_cmp(&$crate::intrinsics::discriminant_value(other)) {
            $crate::option::Option::Some($crate::cmp::Ordering::Equal)=> {
                match (self , other) {
                    (Command::Move {
                        x: x_self, y: y_self,
                    }
                    , Command::Move {
                        x: x_other, y: y_other,
                    }
                    )=>match x_self.partial_cmp(&x_other) {
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal)=> {
                            match y_self.partial_cmp(&y_other) {
                                $crate::option::Option::Some($crate::cmp::Ordering::Equal)=> {
                                    $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                                }
                                c=>return c,
                            }
                        }
                        c=>return c,
                    }
                    , (Command::Do(f0_self, ), Command::Do(f0_other, ))=>match f0_self.partial_cmp(&f0_other) {
                        $crate::option::Option::Some($crate::cmp::Ordering::Equal)=> {
                            $crate::option::Option::Some($crate::cmp::Ordering::Equal)
                        }
                        c=>return c,
                    }
                    , (Command::Jump, Command::Jump)=>$crate::option::Option::Some($crate::cmp::Ordering::Equal), _unused=>$crate::option::Option::Some($crate::cmp::Ordering::Equal)
                }
            }
            c=>return c,
        }
    }
}
impl <> $crate::cmp::Ord for Command< > where {
    fn cmp(&self , other: &Self ) -> $crate::cmp::Ordering {
        match $crate::intrinsics::discriminant_value(self ).cmp(&$crate::intrinsics::discriminant_value(other)) {
            $crate::cmp::Ordering::Equal=> {
                match (self , other) {
                    (Command::Move {
                        x: x_self, y: y_self,
                    }
                    , Command::Move {
                        x: x_other, y: y_other,
                    }
                    )=>match x_self.cmp(&x_other) {
                        $crate::cmp::Ordering::Equal=> {
                            match y_self.cmp(&y_other) {
                                $crate::cmp::Ordering::Equal=> {
                                    $crate::cmp::Ordering::Equal
                                }
                                c=>return c,
                            }
                        }
                        c=>return c,
                    }
                    , (Command::Do(f0_self, ), Command::Do(f0_other, ))=>match f0_self.cmp(&f0_other) {
                        $crate::cmp::Ordering::Equal=> {
                            $crate::cmp::Ordering::Equal
                        }
                        c=>return c,
                    }
                    , (Command::Jump, Command::Jump)=>$crate::cmp::Ordering::Equal, _unused=>$crate::cmp::Ordering::Equal
                }
            }
            c=>return c,
        }
    }
}"#]] ,) ; }
    };
}

test_partial_ord_expand!();