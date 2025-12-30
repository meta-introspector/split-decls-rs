// Generated macro for impl_686 (impl)
macro_rules! Depcrate_base_componentwiseimpl_686 {
() => {
// Module: crate::base::componentwise
// Provides: {"impl_686"}
// Dependencies: {}
# [doc = " # Componentwise operations"] impl < T : Scalar , R1 : Dim , C1 : Dim , SA : Storage < T , R1 , C1 > > Matrix < T , R1 , C1 , SA > { component_binop_impl ! (component_mul , component_mul_mut , component_mul_assign , cmpy , ClosedMulAssign . mul . mul_assign , r"
        Componentwise matrix or vector multiplication.

        # Example

        ```
        # use nalgebra::Matrix2;
        let a = Matrix2::new(0.0, 1.0, 2.0, 3.0);
        let b = Matrix2::new(4.0, 5.0, 6.0, 7.0);
        let expected = Matrix2::new(0.0, 5.0, 12.0, 21.0);

        assert_eq!(a.component_mul(&b), expected);
        ```
        " , r"
        Computes componentwise `self[i] = alpha * a[i] * b[i] + beta * self[i]`.

        # Example
        ```
        # use nalgebra::Matrix2;
        let mut m = Matrix2::new(0.0, 1.0, 2.0, 3.0);
        let a = Matrix2::new(0.0, 1.0, 2.0, 3.0);
        let b = Matrix2::new(4.0, 5.0, 6.0, 7.0);
        let expected = (a.component_mul(&b) * 5.0) + m * 10.0;

        m.cmpy(5.0, &a, &b, 10.0);
        assert_eq!(m, expected);
        ```
        " , r"
        Inplace componentwise matrix or vector multiplication.

        # Example
        ```
        # use nalgebra::Matrix2;
        let mut a = Matrix2::new(0.0, 1.0, 2.0, 3.0);
        let b = Matrix2::new(4.0, 5.0, 6.0, 7.0);
        let expected = Matrix2::new(0.0, 5.0, 12.0, 21.0);

        a.component_mul_assign(&b);

        assert_eq!(a, expected);
        ```
        " ; component_div , component_div_mut , component_div_assign , cdpy , ClosedDivAssign . div . div_assign , r"
        Componentwise matrix or vector division.

        # Example

        ```
        # use nalgebra::Matrix2;
        let a = Matrix2::new(0.0, 1.0, 2.0, 3.0);
        let b = Matrix2::new(4.0, 5.0, 6.0, 7.0);
        let expected = Matrix2::new(0.0, 1.0 / 5.0, 2.0 / 6.0, 3.0 / 7.0);

        assert_eq!(a.component_div(&b), expected);
        ```
        " , r"
        Computes componentwise `self[i] = alpha * a[i] / b[i] + beta * self[i]`.

        # Example
        ```
        # use nalgebra::Matrix2;
        let mut m = Matrix2::new(0.0, 1.0, 2.0, 3.0);
        let a = Matrix2::new(4.0, 5.0, 6.0, 7.0);
        let b = Matrix2::new(4.0, 5.0, 6.0, 7.0);
        let expected = (a.component_div(&b) * 5.0) + m * 10.0;

        m.cdpy(5.0, &a, &b, 10.0);
        assert_eq!(m, expected);
        ```
        " , r"
        Inplace componentwise matrix or vector division.

        # Example
        ```
        # use nalgebra::Matrix2;
        let mut a = Matrix2::new(0.0, 1.0, 2.0, 3.0);
        let b = Matrix2::new(4.0, 5.0, 6.0, 7.0);
        let expected = Matrix2::new(0.0, 1.0 / 5.0, 2.0 / 6.0, 3.0 / 7.0);

        a.component_div_assign(&b);

        assert_eq!(a, expected);
        ```
        " ;) ; # [doc = " Computes the infimum (aka. componentwise min) of two matrices/vectors."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nalgebra::Matrix2;"] # [doc = " let u = Matrix2::new(4.0, 2.0, 1.0, -2.0);"] # [doc = " let v = Matrix2::new(2.0, 4.0, -2.0, 1.0);"] # [doc = " let expected = Matrix2::new(2.0, 2.0, -2.0, -2.0);"] # [doc = " assert_eq!(u.inf(&v), expected)"] # [doc = " ```"] # [inline] # [must_use] pub fn inf (& self , other : & Self) -> OMatrix < T , R1 , C1 > where T : SimdPartialOrd , DefaultAllocator : Allocator < R1 , C1 > , { self . zip_map (other , | a , b | a . simd_min (b)) } # [doc = " Computes the supremum (aka. componentwise max) of two matrices/vectors."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nalgebra::Matrix2;"] # [doc = " let u = Matrix2::new(4.0, 2.0, 1.0, -2.0);"] # [doc = " let v = Matrix2::new(2.0, 4.0, -2.0, 1.0);"] # [doc = " let expected = Matrix2::new(4.0, 4.0, 1.0, 1.0);"] # [doc = " assert_eq!(u.sup(&v), expected)"] # [doc = " ```"] # [inline] # [must_use] pub fn sup (& self , other : & Self) -> OMatrix < T , R1 , C1 > where T : SimdPartialOrd , DefaultAllocator : Allocator < R1 , C1 > , { self . zip_map (other , | a , b | a . simd_max (b)) } # [doc = " Computes the (infimum, supremum) of two matrices/vectors."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nalgebra::Matrix2;"] # [doc = " let u = Matrix2::new(4.0, 2.0, 1.0, -2.0);"] # [doc = " let v = Matrix2::new(2.0, 4.0, -2.0, 1.0);"] # [doc = " let expected = (Matrix2::new(2.0, 2.0, -2.0, -2.0), Matrix2::new(4.0, 4.0, 1.0, 1.0));"] # [doc = " assert_eq!(u.inf_sup(&v), expected)"] # [doc = " ```"] # [inline] # [must_use] pub fn inf_sup (& self , other : & Self) -> (OMatrix < T , R1 , C1 > , OMatrix < T , R1 , C1 >) where T : SimdPartialOrd , DefaultAllocator : Allocator < R1 , C1 > , { (self . inf (other) , self . sup (other)) } # [doc = " Adds a scalar to `self`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nalgebra::Matrix2;"] # [doc = " let u = Matrix2::new(1.0, 2.0, 3.0, 4.0);"] # [doc = " let s = 10.0;"] # [doc = " let expected = Matrix2::new(11.0, 12.0, 13.0, 14.0);"] # [doc = " assert_eq!(u.add_scalar(s), expected)"] # [doc = " ```"] # [inline] # [must_use = "Did you mean to use add_scalar_mut()?"] pub fn add_scalar (& self , rhs : T) -> OMatrix < T , R1 , C1 > where T : ClosedAddAssign , DefaultAllocator : Allocator < R1 , C1 > , { let mut res = self . clone_owned () ; res . add_scalar_mut (rhs) ; res } # [doc = " Adds a scalar to `self` in-place."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nalgebra::Matrix2;"] # [doc = " let mut u = Matrix2::new(1.0, 2.0, 3.0, 4.0);"] # [doc = " let s = 10.0;"] # [doc = " u.add_scalar_mut(s);"] # [doc = " let expected = Matrix2::new(11.0, 12.0, 13.0, 14.0);"] # [doc = " assert_eq!(u, expected)"] # [doc = " ```"] # [inline] pub fn add_scalar_mut (& mut self , rhs : T) where T : ClosedAddAssign , SA : StorageMut < T , R1 , C1 > , { for e in self . iter_mut () { * e += rhs . clone () } } }
};
}
