macro_rules! deps {
    () => {
        VariableDefinition!();
        FieldDefinition!();
        FragmentDefinition!();
        FragmentSpread!();
        Field!();
        InlineFragment!();
    };
}

macro_rules! DirectiveLocation {
    () => {
        deps!();
        # [doc = " Where a directive can apply to."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#DirectiveLocation)."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum DirectiveLocation { # [doc = " A [query](enum.OperationType.html#variant.Query)"] # [doc = " [operation](struct.OperationDefinition.html)."] Query , # [doc = " A [mutation](enum.OperationType.html#variant.Mutation)"] # [doc = " [operation](struct.OperationDefinition.html)."] Mutation , # [doc = " A [subscription](enum.OperationType.html#variant.Subscription)"] # [doc = " [operation](struct.OperationDefinition.html)."] Subscription , # [doc = " A [field](struct.Field.html)."] Field , # [doc = " A [fragment definition](struct.FragmentDefinition.html)."] FragmentDefinition , # [doc = " A [fragment spread](struct.FragmentSpread.html)."] FragmentSpread , # [doc = " An [inline fragment](struct.InlineFragment.html)."] InlineFragment , # [doc = " A [schema](struct.Schema.html)."] Schema , # [doc = " A [scalar](enum.TypeKind.html#variant.Scalar)."] Scalar , # [doc = " An [object](struct.ObjectType.html)."] Object , # [doc = " A [field definition](struct.FieldDefinition.html)."] FieldDefinition , # [doc = " An [input value definition](struct.InputFieldDefinition.html) as the"] # [doc = " arguments of a field but not an input object."] ArgumentDefinition , # [doc = " An [interface](struct.InterfaceType.html)."] Interface , # [doc = " A [union](struct.UnionType.html)."] Union , # [doc = " An [enum](struct.EnumType.html)."] Enum , # [doc = " A [value on an enum](struct.EnumValueDefinition.html)."] EnumValue , # [doc = " An [input object](struct.InputObjectType.html)."] InputObject , # [doc = " An [input value definition](struct.InputValueDefinition.html) on an"] # [doc = " input object but not a field."] InputFieldDefinition , # [doc = " An [variable definition](struct.VariableDefinition.html)."] VariableDefinition , }
    };
}

DirectiveLocation!();