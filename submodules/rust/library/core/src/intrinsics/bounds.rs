mkuse!{use crate :: marker :: PointeeSized ;}
mkitem!{mktrait!{# [doc = " Types with a built-in dereference operator in runtime MIR,"] # [doc = " aka references and raw pointers."] # [doc = ""] # [doc = " # Safety"] # [doc = " Must actually *be* such a type."] pub unsafe trait BuiltinDeref : Sized { type Pointee : PointeeSized ; }}}
mkitem!{mkimpl!{unsafe impl < T : PointeeSized > BuiltinDeref for & mut T { type Pointee = T ; }}}
mkitem!{mkimpl!{unsafe impl < T : PointeeSized > BuiltinDeref for & T { type Pointee = T ; }}}
mkitem!{mkimpl!{unsafe impl < T : PointeeSized > BuiltinDeref for * mut T { type Pointee = T ; }}}
mkitem!{mkimpl!{unsafe impl < T : PointeeSized > BuiltinDeref for * const T { type Pointee = T ; }}}
mkitem!{mktrait!{pub trait ChangePointee < U : PointeeSized > : BuiltinDeref { type Output ; }}}
mkitem!{mkimpl!{impl < 'a , T : PointeeSized + 'a , U : PointeeSized + 'a > ChangePointee < U > for & 'a mut T { type Output = & 'a mut U ; }}}
mkitem!{mkimpl!{impl < 'a , T : PointeeSized + 'a , U : PointeeSized + 'a > ChangePointee < U > for & 'a T { type Output = & 'a U ; }}}
mkitem!{mkimpl!{impl < T : PointeeSized , U : PointeeSized > ChangePointee < U > for * mut T { type Output = * mut U ; }}}
mkitem!{mkimpl!{impl < T : PointeeSized , U : PointeeSized > ChangePointee < U > for * const T { type Output = * const U ; }}}